# poke


Created by [@Andrea-moth](https://github.com/Andrea-moth)

Discord: Ranni simp#2453

**This was coded for linux, it may work on mac but it will break on windows**

---

Video preview to come later

## Index

- [Installation](https://github.com/Andrea-moth/poke/blob/main/README.md#installation)
- [Usage](https://github.com/Andrea-moth/poke/blob/main/README.md#usage)
- [ToDo](https://github.com/Andrea-moth/poke/blob/main/README.md#todo)

## Installation 

```
git clone https://github.com/Andrea-moth/poke

cd poke

sudo mv path-to-poke /usr/bin
```

> If there is a better way of doing this please let me know

You can then delete the rest of the files, please check the [non personal usage section](https://github.com/Andrea-moth/poke/blob/main/README.md#non-personal-usage) to see if it applies to you

## Usage

### Subcommands 

#### c

Creates a C file using 

```C
#include <stdio.h>

int main(){
	return 0;
}
```

##### Arguments 

* name (Default: main.c) ┃ Lets you set the name of the file 
> There is no need to add .c to the end of the file name, poke will do that for you

---

#### cpp

Creates a C++ file using 

```cpp
#include <iostream>

int main(){
	return 0;
}
```

##### Arguments 

* name (Default: main.cpp) ┃ Lets you set the name of the file 
> There is no need to add .cpp to the end of the file name, poke will do that for you

---

#### py

Creates a python file using 

```python
#!/usr/bin/env python

def main():
    pass
    
if __name__ == "__main__":
    main()
```

##### Arguments 

* name (Default: main) ┃ Lets you set the name of the file 

---

#### sh

Creates a bash file using 

```sh
#!/bin/bash
```

##### Arguments 

* name (Default: main) ┃ Lets you set the name of the file 

---

## ToDo 

- [ ] Add more languages


## Non personal usage 

If you use poke in anything other than a personal useage (i.e. Installing on any computer that isn't your own, etc) please also use

```
sudo mv path-to-readme /usr/bin/poke
```
