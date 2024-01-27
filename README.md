# Sprite Packer for Mana Seed Character Base

## About the Tool

This image processing tool is designed to work with the Character Base spritesheet collection by Seliel the Shaper. The collection, tailored for RPGs, farming sims, or adventure games, features a versatile "paper doll" style sprite system. This tool automates tasks like scanning sprite patterns, organizing them into groups, and concatenating them into various formats, which is particularly beneficial for game developers working with these sprite assets.

For more details about the Character Base spritesheet collection, visit [here](https://seliel-the-shaper.itch.io/character-base).

## Configuration

The tool uses `config.toml` for settings:

- `directory_path`: Directory containing sprite files.
- `pattern`: Regex pattern for sprite file names.
- `output_directory`: Directory for output sprites and JSON files.
- `output_format`: Format for concatenating sprites (flat list or matrix).

## Usage

1. Download the latest release from the GitHub project's [Releases page](https://github.com/adaoraul/sprite-packer/releases).
2. Extract the downloaded file to a desired location.
3. Edit the `config.toml` file according to your requirements.
4. Run the executable file for your platform (Windows, MacOS, Linux).

## Output

Outputs concatenated sprites and generates two JSON files:

- `matching_files.json`: Details of matching files.
- `file_descriptions.json`: Descriptions of output files.

## Contributing

Contributions are welcome for bugs and feature requests.

## License

Licensed under [MIT License](LICENSE).
