# -*- coding: utf-8 -*-

import os, json
import sys
from types import SimpleNamespace
import matplotlib.pyplot as plt

path_to_json = sys.argv[1]
json_files = [pos_json for pos_json in os.listdir(path_to_json) if pos_json.endswith('.json')]


runs = {}
for json_file_name in json_files:
    with open(os.path.join(path_to_json, json_file_name)) as json_file:
        json_text = json.load(json_file)
        runs[eval(json_file_name.replace(".json", ""))] = json_text
        
json_keys = next(iter(runs.values())).keys()
json_keys= list(json_keys)

plt.rcParams.update({'font.size': 5})
plot = ['total_requests', 'requests_per_second', 'total_transferred', 'average_number_of_requests_by_a_task',
        'total_requests_succeded', 'total_requests_failed', 'average_succeds_per_task', 'average_failed_per_task']
for key in json_keys:
    figure, axis = plt.subplots(3, 3)
    data = {k: v[key] for k, v in runs.items()}
    data_keys = list(data)
    data_keys.sort()
    
    metrics = [ [ data[i][p] for i in data_keys ] for p in plot ];
    axis[0, 0].set_title(plot[0], fontsize=5)
    axis[0, 0].plot(data_keys, metrics[0])
    axis[0, 1].set_title(plot[1], fontsize=5)
    axis[0, 1].plot(data_keys, metrics[1])
    axis[0, 2].set_title(plot[2], fontsize=5)
    axis[0, 2].plot(data_keys, metrics[2])
    
    axis[1, 0].set_title(plot[3], fontsize=5)
    axis[1, 0].plot(data_keys, metrics[3])
    axis[1, 1].set_title(plot[4], fontsize=5)
    axis[1, 1].plot(data_keys, metrics[4])
    axis[1, 2].set_title(plot[5], fontsize=5)
    axis[1, 2].plot(data_keys, metrics[5])
    
    axis[2, 0].set_title(plot[6], fontsize=5)
    axis[2, 0].plot(data_keys, metrics[6])
    axis[2, 1].set_title(plot[7], fontsize=5)
    axis[2, 1].plot(data_keys, metrics[7])
    
    figure.tight_layout()
    plt.savefig(key+'.png', dpi=1200)
    
    
    
    