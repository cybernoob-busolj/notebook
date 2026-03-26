#!/usr/bin/python3

import re
from lib.core.data import kb
from lib.core.enums import PRIORITY

__priority__ = PRIORITY.NORMAL

def dependencies():
   pass

def tamper(payload, **kwargs):
    print(payload)
    return payload