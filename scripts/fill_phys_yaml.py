#!/usr/bin/env python

import yaml
import sys

data=[i for i in yaml.load_all(open(sys.argv[1]).read())]

data[9]['th1m']=int(sys.argv[2])
data[9]['th1p']=int(sys.argv[3])
data[9]['th2m']=int(sys.argv[4])
data[9]['th2p']=int(sys.argv[5])
data[9]['th3m']=int(sys.argv[6])
data[9]['th3p']=int(sys.argv[7])
data[9]['trg_en']=int(sys.argv[8],2)
print(int(sys.argv[8],2))
yaml.safe_dump_all(data, open(sys.argv[9], 'w'))