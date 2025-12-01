# Face Detection Rust

Aplikasi deteksi wajah menggunakan Rust dengan image processing, dilengkapi dengan API REST dan frontend web modern.

## 🎯 Fitur
- ✅ Upload gambar untuk deteksi wajah (drag & drop atau click)
- ✅ Deteksi multiple wajah dalam satu gambar
- ✅ Visualisasi hasil dengan bounding boxes dan confidence scores
- ✅ Crop wajah yang terdeteksi
- ✅ Download wajah yang tercrop
- ✅ Real-time processing dengan async Rust
- ✅ Modern UI dengan responsive design
- ✅ REST API untuk integrasi
- ✅ Error handling yang comprehensive

## 🏗️ Teknologi
- **Backend**: Rust dengan Actix-web
- **Image Processing**: image crate + imageproc
- **Async Runtime**: Tokio
- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **Face Detection**: Mock implementation (siap untuk integrasi OpenCV/MediaPipe)

## 📋 Prasyarat
- Rust 1.70+ (install dari [rustup.rs](https://rustup.rs))
- Cargo (termasuk dengan Rust)
- Node.js (opsional, untuk development frontend)

## 🚀 Instalasi & Setup

### 1. Clone Repository
```bash
git clone https://github.com/0xHadiRamdhani/rust-face-detect
cd rust-face-detect
```

### 2. Build Project
```bash
cargo build --release
```

### 3. Jalankan Server
```bash
cargo run
```

Server akan berjalan di `http://localhost:8080`

## 📖 Cara Penggunaan

### 1. Akses Web Interface
Buka browser dan aksa `http://localhost:8080`

### 2. Upload Gambar
- **Drag & Drop**: Tarik gambar ke area upload
- **Click to Upload**: Klik area upload dan pilih file
- **Format yang didukung**: JPG, JPEG, PNG (max 10MB)

### 3. Lihat Hasil
- Gambar asli dan hasil deteksi ditampilkan side-by-side
- Bounding boxes berwarna hijau menandai wajah yang terdeteksi
- Informasi detail untuk setiap wajah (posisi, ukuran, confidence)

### 4. Crop Wajah
- Klik tombol "Crop Face" pada setiap wajah untuk crop individual
- Klik tombol "Crop All Faces" untuk crop semua wajah sekaligus
- Download hasil crop sebagai file gambar

## 🔧 API Endpoints

### Upload Image
```http
POST /api/upload
Content-Type: multipart/form-data

Body:
- image: File (JPG/PNG, max 10MB)

Response: 200 OK
{
  "success": true,
  "data": {
    "original_image": "base64_encoded_string",
    "processed_image": "base64_encoded_string_with_boxes",
    "faces": [
      {
        "x": 100,
        "y": 150,
        "width": 80,
        "height": 80,
        "confidence": 0.95
      }
    ],
    "total_faces": 1,
    "processing_time_ms": 150
  }
}
```

### Crop Faces
```http
POST /api/crop
Content-Type: application/json

Body:
{
  "image_data": "base64_encoded_image",
  "faces": [
    {
      "x": 100,
      "y": 150,
      "width": 80,
      "height": 80,
      "confidence": 0.95
    }
  ]
}

Response: 200 OK
{
  "success": true,
  "data": {
    "cropped_faces": ["base64_encoded_cropped_face_1", "base64_encoded_cropped_face_2"]
  }
}
```

### Health Check
```http
GET /api/health

Response: 200 OK
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2024-01-01T00:00:00Z",
    "version": "0.1.0",
    "service": "face-detect-rust"
  }
}
```

## 🏗️ Struktur Project
```
face-detect-rust/
├── src/
│   ├── main.rs              # Entry point & server setup
│   ├── lib.rs               # Library exports
│   ├── api/                 # API endpoints
│   │   ├── health.rs        # Health check
│   │   ├── upload.rs        # File upload handler
│   │   └── crop.rs          # Face cropping
│   ├── detection/           # Face detection logic
│   │   ├── detector.rs      # Main detector
│   │   └── models.rs        # Detection models
│   ├── models/              # Data structures
│   │   ├── api_response.rs  # API response structures
│   │   └── error.rs         # Error types
│   └── utils/               # Utilities
│       └── image.rs         # Image processing utilities
├── static/                  # Frontend files
│   ├── index.html           # Main page
│   ├── css/
│   │   └── style.css        # Styling
│   └── js/
│       └── main.js          # Frontend logic
├── uploads/                 # Temporary upload directory
├── Cargo.toml               # Rust dependencies
├── README.md                # Documentation
└── .gitignore              # Git ignore rules
```

## 🛠️ Development Commands

```bash
# Development run
cargo run

# Build for production
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Check dependencies
cargo audit
```

## 🔧 Konfigurasi

### Environment Variables
- `PORT`: Port server (default: 8080)
- `RUST_LOG`: Level logging (debug, info, warn, error)

### File Size Limits
- Max upload size: 10MB
- Supported formats: JPG, JPEG, PNG

## 🚀 Deployment

### Local Development
```bash
cargo run
```

### Production dengan Docker
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    libopencv-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/face-detect-rust /usr/local/bin/
COPY static ./static
EXPOSE 8080
CMD ["face-detect-rust"]
```

### Cloud Deployment
- **AWS**: EC2, ECS, atau Lambda
- **Google Cloud**: Compute Engine atau Cloud Run
- **Azure**: Container Instances atau App Service

## 🧪 Testing

### Manual Testing
1. Jalankan server: `cargo run`
2. Buka `http://localhost:8080`
3. Upload gambar dengan wajah
4. Verifikasi deteksi berhasil
5. Test fitur cropping

### API Testing
```bash
# Health check
curl http://localhost:8080/api/health

# Upload test (gunakan gambar yang valid)
curl -X POST -F "image=@test.jpg" http://localhost:8080/api/upload
```

## 📝 Catatan Implementasi

### Mock Detection
Saat ini project menggunakan mock face detection untuk demonstrasi. Untuk implementasi nyata:

1. **OpenCV**: Install `opencv` crate dan gunakan Haar cascades
2. **MediaPipe**: Integrasi dengan MediaPipe Rust bindings
3. **Custom Model**: Gunakan model ML yang telah dilatih

### Performance
- Async processing untuk handle multiple requests
- Image resizing untuk optimasi memory
- Base64 encoding untuk kemudahan frontend

### Security
- File type validation
- File size limits
- Input sanitization
- Error message sanitization

## 🤝 Kontribusi
1. Fork repository
2. Buat branch fitur (`git checkout -b feature/amazing-feature`)
3. Commit perubahan (`git commit -m 'Add amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buat Pull Request

## 📄 Lisensi
Project ini open source. Silakan gunakan dan modifikasi sesuai kebutuhan.

## 🐛 Bug Report & Feature Request
Untuk bug report atau feature request, silakan buat issue di repository ini.

## 📞 Kontak
- Email: [hadsxdev@gmail.com]
- GitHub: [0xHadiRamdhani]

---

**Happy Face Detecting!** 🎯✨