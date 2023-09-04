<div align=center>
  <h1>Keep My House (CLI)</h1>
  <p>CLI password manager with encryption: AES256, Salsa20 and Chacha20, with cross platform and exclusive features</p>
  <img width=900px src="https://github.com/aniko33/KeepMyHouse-cli/assets/76649588/2ba0e24f-bfee-4a92-9d83-69a27b698c6c">
</div>

## Features

- AES256 GCM, Salsa20, Chacha20

- Crossplatform (Windows, Linux, MacOS tested)

- Add, remove, modify mode

- Login available with keyfile

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



Choose whether to have a password or a keyfile 

```textile
? What type of login do you want to use?
➤ Password
  File
```



Choose cryptography

```textile
? Which cryptography do you want to use?  
➤ AES256 GCM
  Salsa20
  Chacha20-Poly1305
```



Create a password 

```textile
? Add a password: ******
[Ctrl + r for show password]
```

Or choose the size of the file (the bigger the better)

```textile
? Keyfile size 
➤ 1024
  2048
  4096
```

Choose the name of the keyfile

```textile
choose the name of the file: mykeyfile.private
```



### Open DB

`kmh open mydb.kmh -e <encryption>`

For those who have a keyfile: `kmh open mydb.kmh -e <encryption> --file`

Insert DB password

```textile
? password: ******
[Ctrl + r for show password]
```

Or insert keyfile path

```textile
? Insert keyfile path: mykeyfile.private
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
