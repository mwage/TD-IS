# Library imports
import pandas as pd
import numpy as np
import math
import matplotlib.pyplot as plt
from matplotlib.ticker import LogFormatterSciNotation
from itertools import groupby
from math import prod

class Graph:
    def __init__(self, vertices):
    self.vertices = vertices
