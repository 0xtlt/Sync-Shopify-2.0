# Sync Shopify 2.0

The software:
"Sync Shopify 2.0" which is a tool software which synchronize the files of a given shopify theme folder with one or multiples shopify stores online.

## Installation

You just need to download the file and make it executable.
```bash
chmod +x syncshopify2
```

## The arguments

### `--stores` (*required)

The argument stores is a text of {shopPrefix)={app password}:{theme id} separated by coma (,).

#### Example

```bash
--stores=mystore.myshopify.com=myapppassword:123456789
```

### `--theme-dir` (*required)

The argument theme-dir is the relative path of the theme folder.

#### Example
```bash
--theme-dir=./theme
```

## Usage

```bash
sync-shopify --stores={shopPrefix)={app password}:{theme id} --theme-dir="./"
```

## License

GPL 3.0

## Contributors

  - [Thomas Tastet](https://github.com/0xtlt/)