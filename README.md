# langid: A Fast Language Identification Tool

![GitHub Repo stars](https://img.shields.io/github/stars/eliangonde/langid?style=social) ![GitHub forks](https://img.shields.io/github/forks/eliangonde/langid?style=social) ![GitHub issues](https://img.shields.io/github/issues/eliangonde/langid) ![GitHub license](https://img.shields.io/github/license/eliangonde/langid) 

## Overview

**langid** is a fast and efficient language identification tool, serving as a rewrite of the original `langid/py3langid`. This library provides a robust solution for detecting languages in text data. It is designed for developers, researchers, and anyone needing to quickly identify the language of a given text.

## Features

- **Speed**: Optimized for quick detection, making it suitable for large datasets.
- **Accuracy**: High accuracy rates for a wide range of languages.
- **Lightweight**: Minimal dependencies for easy integration.
- **Flexible**: Works with various input formats and sizes.
- **Easy to Use**: Simple API for quick implementation.

## Topics

This repository covers a variety of topics relevant to language detection:

- detect-language
- detect-languages
- langid
- language-detection
- language-detection-lib
- language-detection-library
- language-detector
- language-identification
- language-recognition
- nlp
- whatlang

## Installation

To install **langid**, you can use pip. Run the following command in your terminal:

```bash
pip install langid
```

## Usage

Here's a simple example of how to use **langid** in your Python project:

```python
import langid

text = "Bonjour tout le monde"
language, confidence = langid.classify(text)

print(f"Detected language: {language} with confidence {confidence}")
```

This code snippet will detect the language of the input text and provide a confidence score.

## API Reference

### `langid.classify(text)`

- **Parameters**: 
  - `text` (str): The text you want to analyze.
  
- **Returns**: 
  - A tuple containing the detected language code and confidence score.

### Supported Languages

**langid** supports a wide range of languages. Here are a few examples:

- English (`en`)
- French (`fr`)
- Spanish (`es`)
- German (`de`)
- Chinese (`zh`)
- And many more...

## Examples

You can find more examples in the [examples directory](https://github.com/eliangonde/langid/examples).

## Testing

To run tests for the **langid** library, use the following command:

```bash
pytest
```

Make sure you have `pytest` installed. You can install it using pip:

```bash
pip install pytest
```

## Contributing

Contributions are welcome! If you want to contribute to **langid**, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your branch to your forked repository.
5. Create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/eliangonde/langid/blob/main/LICENSE) file for details.

## Releases

You can find the latest releases of **langid** [here](https://github.com/eliangonde/langid/releases). Download the appropriate version and follow the instructions for installation.

## Contact

For any questions or suggestions, feel free to open an issue in the repository or contact the maintainer.

## Acknowledgments

Thanks to the contributors and the community for their support in making **langid** a better tool.

## Support

If you find this library useful, please consider giving it a star on GitHub!

## Further Reading

For more detailed information on language identification and natural language processing, consider the following resources:

- [Natural Language Processing with Python](http://www.nltk.org/book/)
- [Deep Learning for Natural Language Processing](https://www.deeplearningbook.org/)
- [The Stanford NLP Group](https://nlp.stanford.edu/)

## Getting Started

To get started, simply clone the repository and run the example scripts:

```bash
git clone https://github.com/eliangonde/langid.git
cd langid
python example.py
```

## Community

Join our community on GitHub and contribute to the project. We welcome discussions, suggestions, and feedback.

## Links

- [Documentation](https://github.com/eliangonde/langid/wiki)
- [Issues](https://github.com/eliangonde/langid/issues)
- [Pull Requests](https://github.com/eliangonde/langid/pulls)

For the latest updates, visit the [Releases section](https://github.com/eliangonde/langid/releases).