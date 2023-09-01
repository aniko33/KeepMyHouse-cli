<div align=center>
  <h1>Keep My House (CLI)</h1>
  <p>AES-encrypted cli password manager, Chacha20 and Salsa20 with exclusive and crossplatform features</p>
  <img width=900px src="https://github.com/aniko33/KeepMyHouse-cli/assets/76649588/2ba0e24f-bfee-4a92-9d83-69a27b698c6c">
</div>

## Features

- AES256 GCM, Salsa20, Chacha20

- Add, remove, modify mode

- Copy password to clipboard

- Only offline mode

- No-SQL database (using JSON format)

## Installation

### From source

```bash
git clone https://github.com/aniko33/KeepMyHouse-cli && cd KeepMyHouse-cli
chmod +x build.sh && ./build.sh
sudo mv kmh-cli /usr/bin/kmh
```

## Usage

```bash
Usage: kmh-cli <COMMAND>

Commands:
  init  Create new database
  open  Open a database
  list  List of elements
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Init new DB

`kmh init mydb.kmh`

Choose cryptography

```textile
? Which cryptography do you want to use?  
➤ AES256 GCM
  Salsa20
  Chacha20-Poly1305
```

Login using DB password

```textile
? Add a password: ******
[Ctrl + r for show password]
```

### Open DB

`kmh open mydb.kmh -e <encryption>`

Insert DB password

```textile
? password: ******
[Ctrl + r for show password]
```

Welcome to the main menu, enjoy

```textile
ID     Title       Username       Password         Notes
--     -------     ----------     ------------     -----
0      mytitle     myusername     ************          

? What do you want to do?  
➤ Add
  Remove
  Modify
  Show password
  Copy password
  Save
```

## Encryption algorithms

`kmh list -e`

```textile
   Encryption list
   |
   ├── [ aes256 ]
   |
   ├── [ salsa20 ]
   |
   └── [ chacha20 ]
```

## License

This repository is licensed under GPL v3.0 for information look at the file ["LICENSE"](LICENSE)

## Contributors

<a href="https://github.com/aniko33/KeepMyHouse-cli/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=aniko33/KeepMyHouse-cli"/>
</a>
