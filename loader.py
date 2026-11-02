#!/usr/bin/env python3

from pxr.Usd import Stage
from sys import argv, exit

def parse_args():
    if len(argv) <= 1:
        print(f"usage: {argv[0]} file0.usd [file1.usd ...]")
        exit(1)

    for file in argv[1:]:
        yield file


def parse_usda(file_name: str):
    print(f"parse this USDA {file_name=}")
    stage = Stage.Open(file_name)
    print(f"{stage=}\n{type(stage)=}")

    print("--- Scene Hierarchy ---")

    trav = stage.Traverse()
    print(f"{trav=} {type(trav)=}")

    for prim in stage.Traverse():
        print(f"{prim=} {type(prim)=}")
        type_name = prim.GetTypeName()
        print(f"{type_name=} {type(type_name)=}")
        if type_name == "Mesh":
            for attr in  prim.GetAttributes():
                print(f"  {attr=} {type(attr)=}")
                attr_val = attr.Get()
                if attr_val is not None:
                    print(f"    {attr_val=} {type(attr_val)=}")


for file_name in parse_args():
    parse_usda(file_name)