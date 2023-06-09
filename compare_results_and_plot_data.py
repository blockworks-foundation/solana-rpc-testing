# -*- coding: utf-8 -*-

import os, json
import sys
from types import SimpleNamespace
import matplotlib.pyplot as plt

path_1 = sys.argv[1]
path_2 = sys.argv[2]
path_3 = sys.argv[3]

def extract_data(path):
    json_files = [pos_json for pos_json in os.listdir(path) if pos_json.endswith('.json')]
    runs = {}
    for json_file_name in json_files:
        with open(os.path.join(path, json_file_name)) as json_file:
            json_text = json.load(json_file)
            runs[eval(json_file_name.replace(".json", ""))] = json_text
    return runs

def get_keys(runs):
    json_keys = next(iter(runs.values())).keys()
    json_keys= list(json_keys)
    return json_keys

def get_metrics(key, runs, plot):
    data = {k: v[key] for k, v in runs.items()}
    data_keys = list(data)
    data_keys.sort()
    metrics = [ [ data[i][p] for i in data_keys ] for p in plot ];
    return [data_keys, metrics]
    

runs_1 = extract_data(path_1)
runs_2 = extract_data(path_2)
runs_3 = extract_data(path_3)

keysA = get_keys(runs_1)
keysB = get_keys(runs_2)

assert(keysA.sort() == keysB.sort())

plt.rcParams.update({'font.size': 5})
plot = ['total_requests', 'requests_per_second', 'total_transferred', 'average_number_of_requests_by_a_task',
        'total_requests_succeded', 'total_requests_failed', 'average_succeds_per_task', 'average_failed_per_task']
for key in keysA:
    figure, axis = plt.subplots(3, 3)
    [data_keys_1, metrics_1] = get_metrics(key, runs_1, plot)
    [data_keys_2, metrics_2] = get_metrics(key, runs_2, plot) 
    [data_keys_3, metrics_3] = get_metrics(key, runs_3, plot) 

    axis[0, 0].set_title(plot[0], fontsize=5)
    axis[0, 0].plot(data_keys_1, metrics_1[0], 'r', label=path_1)
    axis[0, 0].plot(data_keys_2, metrics_2[0], 'b', label=path_2)
    axis[0, 0].plot(data_keys_3, metrics_3[0], 'g', label=path_3)
    
    axis[0, 1].set_title(plot[1], fontsize=5)
    axis[0, 1].plot(data_keys_1, metrics_1[1], 'r')
    axis[0, 1].plot(data_keys_2, metrics_2[1], 'b')
    axis[0, 1].plot(data_keys_3, metrics_3[1], 'g')
    
    axis[0, 2].set_title(plot[2], fontsize=5)
    axis[0, 2].plot(data_keys_1, metrics_1[2], 'r')
    axis[0, 2].plot(data_keys_2, metrics_2[2], 'b')
    axis[0, 2].plot(data_keys_3, metrics_3[2], 'g')
    
    
    axis[1, 0].set_title(plot[3], fontsize=5)
    axis[1, 0].plot(data_keys_1, metrics_1[3], 'r')
    axis[1, 0].plot(data_keys_2, metrics_2[3], 'b')
    axis[1, 0].plot(data_keys_3, metrics_3[3], 'g')
    
    axis[1, 1].set_title(plot[4], fontsize=5)
    axis[1, 1].plot(data_keys_1, metrics_1[4], 'r')
    axis[1, 1].plot(data_keys_2, metrics_2[4], 'b')
    axis[1, 1].plot(data_keys_3, metrics_3[4], 'g')
    
    axis[1, 2].set_title(plot[5], fontsize=5)
    axis[1, 2].plot(data_keys_1, metrics_1[5], 'r')
    axis[1, 2].plot(data_keys_2, metrics_2[5], 'b')
    axis[1, 2].plot(data_keys_3, metrics_3[5], 'g')
    
    
    axis[2, 0].set_title(plot[6], fontsize=5)
    axis[2, 0].plot(data_keys_1, metrics_1[6], 'r')
    axis[2, 0].plot(data_keys_2, metrics_2[6], 'b')
    axis[2, 0].plot(data_keys_3, metrics_3[6], 'g')
    
    axis[2, 1].set_title(plot[7], fontsize=5)
    axis[2, 1].plot(data_keys_1, metrics_1[7], 'r')
    axis[2, 1].plot(data_keys_2, metrics_2[7], 'b')
    axis[2, 1].plot(data_keys_3, metrics_3[7], 'g')
    
    handles, labels = axis[0, 0].get_legend_handles_labels()
    figure.legend(handles, labels, loc='upper left')
    
    figure.tight_layout()
    plt.savefig(key+'.png', dpi=1200)
