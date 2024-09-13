# radio-ctl

This is a simple command line tool to control the radios on a Windows computer.

It can be used to turn the radios on or off, or to query the current state of the radios.

## Usage

List radios:
```shell
radio-ctl.exe
```
or
```shell
radio-ctl.exe --list
```

Turn Bluetooth radios on:
```shell
radio-ctl.exe on --kind=bluetooth
```

Turn Wi-Fi radios off:
```shell
radio-ctl.exe off --kind=wifi
```

Turn specific radio on:
```shell
radio-ctl.exe on --name="Builtin Wi-Fi"
```