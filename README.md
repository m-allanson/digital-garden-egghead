# Digital Garden

A CLI tool for the creation and maintenance of Digital Gardens.

## Commands

### Setting the garden path

```shell
GARDEN_PATH=~/github/my-digital-garden garden write
garden -p ~/github/my-digital-garden write
garden --garden-path ~/github/my-digital-garden write
```

### write

Open a new file to write in our digital garden. Since we don't necessarily know what we want to title what we're writing, we'll eave the title as optinal and ask the user for it later if they don't provide it up-front.

```shell
gd write
gd write -t "Some title"
```