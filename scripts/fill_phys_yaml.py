#!/usr/bin/env python

import yaml
import sys

data=[i for i in yaml.load_all(open(sys.argv[1]).read())]
print(data[9])
data[9]['th1m']=sys.argv[2]
data[9]['th1p']=sys.argv[3]
data[9]['th2m']=sys.argv[4]
data[9]['th2p']=sys.argv[5]
data[9]['th3m']=sys.argv[6]
data[9]['th3p']=sys.argv[7]
data[9]['trg_en']=int(sys.argv[8],2)
print(int(sys.argv[8],2))
yaml.safe_dump_all(data, open(sys.argv[9], 'w'))