## Requirements

- Rust and Cargo installed
- An API token for the Groq API

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/90th/speech-translator-rs.git
    cd speech-translator-rs
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

3. Set up environment variables:
    ```sh
    cp .env.example .env
    # Edit .env to include your API token
    ```

## Usage

1. Run the application:
    ```sh
    cargo run
    ```

2. Follow the prompts to start and stop recording audio.

## Configuration

- Edit the `.env` file to include your Groq API token:
    ```env
    API_TOKEN=your_api_token_here
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [CPAL](https://github.com/RustAudio/cpal) for audio input
- [Hound](https://github.com/ruuda/hound) for WAV file handling
- [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- [dotenv](https://github.com/dotenv-rs/dotenv) for environment variable support
