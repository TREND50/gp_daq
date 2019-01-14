#!/usr/bin/env python

import yaml
import sys

data=[i for i in yaml.load_all(open(sys.argv[1]).read())]

data[9]['th1m']=float(sys.argv[2])
data[9]['th1p']=float(sys.argv[3])
data[9]['th2m']=float(sys.argv[4])
data[9]['th2p']=float(sys.argv[5])
data[9]['th3m']=float(sys.argv[6])
data[9]['th3p']=float(sys.argv[7])
data[9]['trg_en']=int(sys.argv[8],2)
print(int(sys.argv[8],2))
yaml.safe_dump_all(data, open(sys.argv[9], 'w'))
