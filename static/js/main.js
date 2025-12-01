// Global variables
let selectedFile = null;
let currentResult = null;

// DOM elements
const uploadArea = document.getElementById('uploadArea');
const fileInput = document.getElementById('fileInput');
const uploadBtn = document.getElementById('uploadBtn');
const resetBtn = document.getElementById('resetBtn');
const retryBtn = document.getElementById('retryBtn');
const resultsSection = document.getElementById('resultsSection');
const loadingSection = document.getElementById('loadingSection');
const errorSection = document.getElementById('errorSection');

// Initialize the application
document.addEventListener('DOMContentLoaded', function () {
    initializeEventListeners();
    checkServerHealth();
});

function initializeEventListeners() {
    // Upload area events
    uploadArea.addEventListener('click', () => fileInput.click());
    uploadArea.addEventListener('dragover', handleDragOver);
    uploadArea.addEventListener('dragleave', handleDragLeave);
    uploadArea.addEventListener('drop', handleDrop);

    // File input change
    fileInput.addEventListener('change', handleFileSelect);

    // Button events
    uploadBtn.addEventListener('click', handleUpload);
    resetBtn.addEventListener('click', resetApplication);
    retryBtn.addEventListener('click', resetApplication);
}

function handleDragOver(e) {
    e.preventDefault();
    uploadArea.classList.add('drag-over');
}

function handleDragLeave(e) {
    e.preventDefault();
    uploadArea.classList.remove('drag-over');
}

function handleDrop(e) {
    e.preventDefault();
    uploadArea.classList.remove('drag-over');

    const files = e.dataTransfer.files;
    if (files.length > 0) {
        handleFile(files[0]);
    }
}

function handleFileSelect(e) {
    const files = e.target.files;
    if (files.length > 0) {
        handleFile(files[0]);
    }
}

function handleFile(file) {
    // Validate file
    if (!validateFile(file)) {
        return;
    }

    selectedFile = file;

    // Show file preview
    const reader = new FileReader();
    reader.onload = function (e) {
        uploadArea.innerHTML = `
            <div class="file-preview">
                <img src="${e.target.result}" alt="Preview">
                <div class="file-info">
                    <p><strong>${file.name}</strong></p>
                    <p>${formatFileSize(file.size)}</p>
                </div>
            </div>
        `;
        uploadBtn.disabled = false;
    };
    reader.readAsDataURL(file);
}

function validateFile(file) {
    // Check file type
    const validTypes = ['image/jpeg', 'image/jpg', 'image/png'];
    if (!validTypes.includes(file.type)) {
        showError('Please select a valid image file (JPG, JPEG, or PNG)');
        return false;
    }

    // Check file size (10MB limit)
    const maxSize = 10 * 1024 * 1024; // 10MB
    if (file.size > maxSize) {
        showError('File size must be less than 10MB');
        return false;
    }

    return true;
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

async function handleUpload() {
    if (!selectedFile) {
        showError('Please select an image file');
        return;
    }

    // Show loading state
    showLoading();

    try {
        // Create form data
        const formData = new FormData();
        formData.append('image', selectedFile);

        // Upload image
        const response = await fetch('/api/upload', {
            method: 'POST',
            body: formData
        });

        const result = await response.json();

        if (result.success) {
            displayResults(result.data);
        } else {
            showError(result.error || 'Upload failed');
        }
    } catch (error) {
        console.error('Upload error:', error);
        showError('Failed to upload image. Please try again.');
    } finally {
        hideLoading();
    }
}

function displayResults(data) {
    currentResult = data;

    // Update summary
    document.getElementById('totalFaces').textContent = data.total_faces;
    document.getElementById('processingTime').textContent = data.processing_time_ms + 'ms';

    // Display images
    document.getElementById('originalImage').src = data.original_image;
    document.getElementById('processedImage').src = data.processed_image;

    // Display faces list
    const facesList = document.getElementById('facesList');
    facesList.innerHTML = '';

    if (data.faces.length === 0) {
        facesList.innerHTML = '<p class="no-faces">No faces detected in this image</p>';
    } else {
        data.faces.forEach((face, index) => {
            const faceItem = document.createElement('div');
            faceItem.className = 'face-item';
            faceItem.innerHTML = `
                <div class="face-number">Face ${index + 1}</div>
                <div class="face-coords">
                    Position: (${face.x}, ${face.y})<br>
                    Size: ${face.width}×${face.height}
                </div>
                <div class="face-confidence">
                    Confidence: ${(face.confidence * 100).toFixed(1)}%
                </div>
                <button class="btn btn-small crop-btn" onclick="cropFace(${index})">
                    Crop Face
                </button>
            `;
            facesList.appendChild(faceItem);
        });

        // Add crop all button if multiple faces
        if (data.faces.length > 1) {
            const cropAllBtn = document.createElement('button');
            cropAllBtn.className = 'btn btn-primary crop-all-btn';
            cropAllBtn.textContent = 'Crop All Faces';
            cropAllBtn.onclick = cropAllFaces;
            facesList.appendChild(cropAllBtn);
        }
    }

    // Show results section
    showResults();
}

function showResults() {
    hideAllSections();
    resultsSection.style.display = 'block';
}

function showLoading() {
    hideAllSections();
    loadingSection.style.display = 'block';
    uploadBtn.disabled = true;
}

function hideLoading() {
    uploadBtn.disabled = false;
}

function showError(message) {
    hideAllSections();
    document.getElementById('errorMessage').textContent = message;
    errorSection.style.display = 'block';
}

function hideAllSections() {
    resultsSection.style.display = 'none';
    loadingSection.style.display = 'none';
    errorSection.style.display = 'none';
}

function resetApplication() {
    selectedFile = null;
    currentResult = null;
    fileInput.value = '';
    uploadBtn.disabled = true;

    // Reset upload area
    uploadArea.innerHTML = `
        <div class="upload-content">
            <div class="upload-icon">📷</div>
            <h3>Drop your image here</h3>
            <p>or <span class="upload-link">click to upload</span></p>
            <small>Supports JPG, PNG (max 10MB)</small>
        </div>
        <input type="file" id="fileInput" accept="image/*" hidden>
    `;

    // Re-attach event listeners
    const newFileInput = uploadArea.querySelector('#fileInput');
    newFileInput.addEventListener('change', handleFileSelect);
    uploadArea.addEventListener('click', () => newFileInput.click());

    hideAllSections();
    uploadArea.parentElement.style.display = 'block';
}

async function checkServerHealth() {
    try {
        const response = await fetch('/api/health');
        const result = await response.json();

        if (result.success && result.data.status === 'healthy') {
            console.log('Server is healthy');
        } else {
            console.warn('Server health check failed');
        }
    } catch (error) {
        console.error('Health check failed:', error);
    }
}

// Add some utility functions for better UX
function addKeyboardShortcuts() {
    document.addEventListener('keydown', function (e) {
        // Ctrl/Cmd + U to upload
        if ((e.ctrlKey || e.metaKey) && e.key === 'u') {
            e.preventDefault();
            fileInput.click();
        }

        // Escape to reset
        if (e.key === 'Escape') {
            resetApplication();
        }
    });
}

// Initialize keyboard shortcuts
addKeyboardShortcuts();

// Add some visual feedback for better UX
function addVisualEffects() {
    // Add hover effects to buttons
    const buttons = document.querySelectorAll('.btn');
    buttons.forEach(button => {
        button.addEventListener('mouseenter', function () {
            this.style.transform = 'translateY(-2px)';
        });

        button.addEventListener('mouseleave', function () {
            this.style.transform = 'translateY(0)';
        });
    });
}

// Initialize visual effects
addVisualEffects();

// Cropping functions
async function cropFace(faceIndex) {
    if (!currentResult || !currentResult.faces[faceIndex]) {
        showError('No face data available for cropping');
        return;
    }

    showLoading();

    try {
        const face = currentResult.faces[faceIndex];
        const requestData = {
            image_data: currentResult.original_image,
            faces: [face]
        };

        const response = await fetch('/api/crop', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        const result = await response.json();

        if (result.success && result.data.cropped_faces.length > 0) {
            displayCroppedFace(result.data.cropped_faces[0], faceIndex + 1);
        } else {
            showError('Failed to crop face');
        }
    } catch (error) {
        console.error('Crop error:', error);
        showError('Failed to crop face. Please try again.');
    } finally {
        hideLoading();
    }
}

async function cropAllFaces() {
    if (!currentResult || currentResult.faces.length === 0) {
        showError('No faces available for cropping');
        return;
    }

    showLoading();

    try {
        const requestData = {
            image_data: currentResult.original_image,
            faces: currentResult.faces
        };

        const response = await fetch('/api/crop', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        const result = await response.json();

        if (result.success && result.data.cropped_faces.length > 0) {
            displayAllCroppedFaces(result.data.cropped_faces);
        } else {
            showError('Failed to crop faces');
        }
    } catch (error) {
        console.error('Crop all error:', error);
        showError('Failed to crop faces. Please try again.');
    } finally {
        hideLoading();
    }
}

function displayCroppedFace(croppedImageData, faceNumber) {
    // Create modal or popup to show cropped face
    const modal = document.createElement('div');
    modal.className = 'crop-modal';
    modal.innerHTML = `
        <div class="crop-modal-content">
            <div class="crop-modal-header">
                <h3>Cropped Face ${faceNumber}</h3>
                <button class="crop-modal-close" onclick="this.closest('.crop-modal').remove()">×</button>
            </div>
            <div class="crop-modal-body">
                <img src="${croppedImageData}" alt="Cropped Face ${faceNumber}">
            </div>
            <div class="crop-modal-footer">
                <button class="btn btn-primary" onclick="downloadCroppedFace('${croppedImageData}', 'face_${faceNumber}.jpg')">
                    Download
                </button>
                <button class="btn btn-secondary" onclick="this.closest('.crop-modal').remove()">
                    Close
                </button>
            </div>
        </div>
    `;

    document.body.appendChild(modal);

    // Add event listener to close modal when clicking outside
    modal.addEventListener('click', function (e) {
        if (e.target === modal) {
            modal.remove();
        }
    });
}

function displayAllCroppedFaces(croppedImages) {
    // Create modal to show all cropped faces
    const modal = document.createElement('div');
    modal.className = 'crop-modal';

    let imagesHtml = '';
    croppedImages.forEach((image, index) => {
        imagesHtml += `
            <div class="cropped-face-item">
                <img src="${image}" alt="Cropped Face ${index + 1}">
                <div class="cropped-face-info">
                    <p>Face ${index + 1}</p>
                    <button class="btn btn-small" onclick="downloadCroppedFace('${image}', 'face_${index + 1}.jpg')">
                        Download
                    </button>
                </div>
            </div>
        `;
    });

    modal.innerHTML = `
        <div class="crop-modal-content">
            <div class="crop-modal-header">
                <h3>All Cropped Faces</h3>
                <button class="crop-modal-close" onclick="this.closest('.crop-modal').remove()">×</button>
            </div>
            <div class="crop-modal-body">
                <div class="cropped-faces-grid">
                    ${imagesHtml}
                </div>
            </div>
            <div class="crop-modal-footer">
                <button class="btn btn-primary" onclick="downloadAllCroppedFaces(${JSON.stringify(croppedImages)})">
                    Download All
                </button>
                <button class="btn btn-secondary" onclick="this.closest('.crop-modal').remove()">
                    Close
                </button>
            </div>
        </div>
    `;

    document.body.appendChild(modal);

    // Add event listener to close modal when clicking outside
    modal.addEventListener('click', function (e) {
        if (e.target === modal) {
            modal.remove();
        }
    });
}

function downloadCroppedFace(imageData, filename) {
    const link = document.createElement('a');
    link.href = imageData;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

function downloadAllCroppedFaces(croppedImages) {
    croppedImages.forEach((image, index) => {
        setTimeout(() => {
            downloadCroppedFace(image, `face_${index + 1}.jpg`);
        }, index * 200); // Delay between downloads
    });
}