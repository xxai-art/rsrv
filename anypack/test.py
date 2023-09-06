#!/usr/bin/env python

a = 128
for i in range(a):
  print("tuple!(%s);" % ",".join(tuple(map(str, range(i)))))
