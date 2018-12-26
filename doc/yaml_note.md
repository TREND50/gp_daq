# A note on how to read YAML file in python
Make sure python-yaml is install:
either by use the ubuntu package manager:
```
sudo apt install python-yaml python3-yaml
```
or by using the pip:
```
sudo pip install yaml
sudo pip3 install yaml
```
Use yaml in python: For single-doc yaml file (actual not our case)
```
$> ipython
import yaml
data=yaml.load(open('somefilename.yaml').read())
print(data)
```
For multiple-doc yaml file (our case, different docs are separated by ---)

```
$> ipython
import yaml
data=[i for i in yaml.load_all(open('somefilename.yaml').read())]
print(data[0])
print(data[1])
...
```
