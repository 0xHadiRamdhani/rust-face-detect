# Frontend Design - Face Detection Rust

## UI/UX Design Overview
Aplikasi ini akan memiliki interface yang clean dan modern dengan fokus pada kemudahan penggunaan untuk upload gambar dan melihat hasil deteksi wajah.

## Layout Structure

### Main Page Layout
```
+----------------------------------+
|  Face Detection Rust             |
|  [Logo/Title]                    |
+----------------------------------+
|                                  |
|  +----------------------------+  |
|  |  📷 Drop Image Here        |  |
|  |  or click to upload        |  |
|  +----------------------------+  |
|                                  |
|  [Upload Button]                 |
|                                  |
+----------------------------------+
|  Results Area                    |
|  +----------------------------+  |
|  |  Original Image | Processed |  |
|  |                  |  Image    |  |
|  +----------------------------+  |
|  |  Detected Faces: 3           |  |
|  |  - Face 1: (x:100, y:150)    |  |
|  |  - Face 2: (x:250, y:200)    |  |
|  |  - Face 3: (x:400, y:180)    |  |
|  +----------------------------+  |
+----------------------------------+
```

## Component Details

### 1. Upload Area
- **Drag & Drop Zone**: Area besar dengan border dashed untuk drag & drop
- **Click to Upload**: Alternative untuk klik manual
- **File Preview**: Thumbnail preview setelah file dipilih
- **Supported Formats**: Info JPEG, PNG, max 5MB

### 2. Results Display
- **Image Comparison**: Side-by-side original vs processed
- **Bounding Boxes**: Visual overlay pada gambar yang terdeteksi
- **Face Details**: Koordinat dan confidence score
- **Download Options**: Download cropped faces

### 3. Status & Feedback
- **Loading Spinner**: Saat processing
- **Progress Bar**: Upload progress
- **Error Messages**: User-friendly error handling
- **Success Messages**: Konfirmasi hasil

## Color Scheme
```css
:root {
  --primary-color: #2563eb;      /* Blue */
  --secondary-color: #64748b;    /* Gray */
  --success-color: #10b981;      /* Green */
  --error-color: #ef4444;        /* Red */
  --warning-color: #f59e0b;      /* Yellow */
  --background: #ffffff;           /* White */
  --surface: #f8fafc;            /* Light gray */
  --text-primary: #1e293b;       /* Dark gray */
  --text-secondary: #64748b;       /* Medium gray */
}
```

## Responsive Design
- **Mobile First**: Design untuk mobile 320px ke atas
- **Breakpoints**: 640px (tablet), 1024px (desktop)
- **Flexible Layout**: Grid dan flexbox untuk adaptasi
- **Touch Friendly**: Large tap targets untuk mobile

## Interactive Elements

### Upload Zone States
1. **Default**: Border abu-abu dengan icon upload
2. **Drag Over**: Border biru dengan background highlight
3. **Uploading**: Progress bar dengan spinner
4. **Success**: Border hijau dengan checkmark
5. **Error**: Border merah dengan error message

### Image Viewer
- **Zoom**: Hover untuk zoom pada gambar
- **Pan**: Drag untuk navigasi gambar besar
- **Toggle**: Switch antara original dan processed
- **Fullscreen**: Mode fullscreen untuk detail

## Accessibility Features
- **Keyboard Navigation**: Tab order yang logical
- **Screen Reader**: ARIA labels untuk semua elements
- **Alt Text**: Deskripsi untuk semua images
- **Color Contrast**: WCAG 2.1 AA compliance
- **Focus Indicators**: Visible focus states

## JavaScript Functionality

### Core Features
1. **File Upload**: AJAX upload dengan progress tracking
2. **Image Preview**: Client-side preview sebelum upload
3. **Drag & Drop**: Native HTML5 drag & drop API
4. **Canvas Drawing**: Draw bounding boxes pada gambar
5. **Error Handling**: Try-catch untuk semua operations

### API Integration
```javascript
// Upload function
async function uploadImage(file) {
  const formData = new FormData();
  formData.append('image', file);
  
  try {
    const response = await fetch('/api/upload', {
      method: 'POST',
      body: formData
    });
    
    const result = await response.json();
    displayResults(result);
  } catch (error) {
    displayError(error.message);
  }
}
```

### Canvas Drawing
```javascript
// Draw bounding boxes
function drawBoundingBoxes(canvas, faces) {
  const ctx = canvas.getContext('2d');
  ctx.strokeStyle = '#10b981';
  ctx.lineWidth = 2;
  
  faces.forEach(face => {
    ctx.strokeRect(face.x, face.y, face.width, face.height);
    
    // Add confidence label
    ctx.fillStyle = '#10b981';
    ctx.fillText(
      `Confidence: ${(face.confidence * 100).toFixed(1)}%`,
      face.x,
      face.y - 5
    );
  });
}
```

## Performance Optimization
- **Lazy Loading**: Load images on demand
- **Image Compression**: Client-side compression sebelum upload
- **Caching**: Cache hasil deteksi untuk images yang sama
- **Debouncing**: Prevent excessive API calls
- **Virtual Scrolling**: Untuk banyak hasil deteksi

## Browser Compatibility
- **Modern Browsers**: Chrome 80+, Firefox 75+, Safari 13+
- **ES6+ Features**: Async/await, fetch API
- **CSS Grid**: Layout dengan fallback
- **Canvas 2D**: Drawing API support
- **File API**: File upload support

## Security Considerations
- **File Validation**: Client-side type dan size check
- **XSS Prevention**: Sanitize user input
- **CSRF Protection**: Token-based protection
- **Content Security**: Proper CSP headers
- **HTTPS**: Secure transmission