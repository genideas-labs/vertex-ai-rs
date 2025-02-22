# vertex-ai-rs

## Introduction
This project is designed to provide a Rust library for interacting with Google Vertex AI, allowing users to leverage the power of Vertex AI's machine learning capabilities within their Rust applications. The library aims to simplify the process of integrating Vertex AI into Rust projects, making it easier for developers to build and deploy machine learning models.

## Installation Guide
To install this project, follow these steps:
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/vertex-ai-rs.git
   ```
2. Navigate to the project directory:
   ```bash
   cd vertex-ai-rs
   ```
3. Install the required dependencies:
   ```bash
   cargo build
   ```

## Expected Running Examples
To run the project, use the following command:
```bash
cargo run
```

### Example Output
Upon running the project, you can expect the following output:
```
Raw response: {
  "candidates": [
    {
      "content": {
        "role": "model",
        "parts": [
          {
            "text": "Hi there! I'm happy to help in any way I can. Here are some of the things I can assist you with: [list of capabilities]. Please let me know how I can assist you today."
          }
        ]
      }
    }
  ]
}
```

## Basic Contents
- **Usage**: Instructions on how to use the project.
- **Contributing**: Guidelines for contributing to the project.
- **License**: Information about the project's license.

## 서비스 계정 설정 방법

1. [Google Cloud Console](https://console.cloud.google.com/)에 접속
2. 프로젝트 선택 또는 생성
3. "IAM 및 관리" > "서비스 계정" 이동
4. "서비스 계정 만들기" 클릭
5. 필요한 정보 입력:
   - 이름: 원하는 이름 (예: vertex-ai-chat)
   - 역할: Vertex AI User
6. "키 만들기" > "JSON" 선택
7. 다운로드된 JSON 파일을 프로젝트 루트의 `credentials.json`으로 저장

## 문제 해결

- **인증 오류**: credentials.json이 올바른 위치에 있고 올바른 권한이 부여되었는지 확인
- **API 오류**: Vertex AI API가 프로젝트에서 활성화되어 있는지 확인
- **빌드 오류**: Rust가 올바르게 설치되어 있는지 확인

### 일반적인 오류 해결

1. `cargo build` 실패
   - Rust가 최신 버전인지 확인: `rustup update`
   - 의존성 패키지 업데이트: `cargo update`

2. 실행 시 인증 오류
   - credentials.json 파일이 올바른 위치에 있는지 확인
   - 서비스 계정에 필요한 권한이 있는지 확인

## 라이선스

MIT

## 기여

버그 리포트, 기능 요청 및 풀 리퀘스트를 환영합니다.