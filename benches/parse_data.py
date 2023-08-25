#!/usr/bin/env python3

import argparse
import re
from glob import glob
import statistics
import matplotlib.pyplot as plt
import numpy as np


def atoi(text):
    return int(text) if text.isdigit() else text


def natural_keys(text):
    return [atoi(c) for c in re.split(r'(\d+)', text)]


def parse_coap_mqtt(dir: str, use_pairs=True):
    pairs = [(5, 3), (6, 4), (7, 5), (8, 6),
             (9, 7), (10, 8), (11, 9), (12, 10)]
    results = {}
    result_files = sorted(glob("{}/*.txt".format(dir)), key=natural_keys)
    for result_file in result_files:
        (n, t) = (int(result_file.split("_")[2]), int(
            result_file.split("_")[3]))
        if use_pairs and (n, t) not in pairs:
            continue
        row = "$(n = {}, t = {})$".format(n, t)
        results[row] = []

        with open(result_file) as f:
            for line in f.readlines():
                if "Duration: " in line:
                    duration = line.split(" ")[1][:-1]
                    if duration[-2:] == "ms":
                        results[row].append(float(duration[:-2]))
                    elif duration[-1:] == "s":
                        results[row].append(float(duration[:-1]) * 1000)
                    else:
                        raise ValueError(duration)
    return results


def parse_microbenchmarks(dir: str, bench: str, use_pairs=True):
    # TODO: Update nodes list to align with device setup
    nodes = {"node1": "RPi 3B+", "node7": "RPi 2B", "node12": "RPi Zero"}
    # TODO: Update iterations to align with experiment setup
    iterations = 100000
    n = 12
    results = {}

    pairs = [(5, 3), (6, 4), (7, 5), (8, 6),
             (9, 7), (10, 8), (11, 9), (12, 10)]

    if bench in ["eval", "nomac"]:
        for node in nodes:
            row = nodes[node]
            if row not in results:
                results[row] = []
            with open("microbenchmark_data/{}/eval_{}_iterations{}.txt".format(node, iterations, "_nomac" if bench == "nomac" else "")) as f:
                duration = f.read()[:-1]
                if duration[-2:] == "ms":
                    results[row].append(float(duration[:-2]))
                elif duration[-1:] == "s":
                    results[row].append(float(duration[:-1]) * 1000)
                else:
                    raise ValueError(duration)
        return results

    for i in range(5, n+1):
        for j in range(2, -1, -1):
            for node in nodes:
                if use_pairs and (i, i-j) not in pairs:
                    continue
                row = "{} $\mathsf{}$ $(n = {}, t = {})$".format(nodes[node],
                                                                 r"{Gen}" if bench == "init" else r"{Recon}", i, i-j)
                if row not in results:
                    results[row] = []
                #with open("microbenchmark_data/{}/{}_{}_{}_{}.txt".format(node, bench, iterations, i, i-j)) as f:
                with open(f"microbenchmark_data/{node}/{bench}_{iterations}_{i}_{i-j}.txt") as f:
                    duration = f.read()[:-1]
                    if duration[-2:] == "ms":
                        results[row].append(float(duration[:-2]))
                    elif duration[-2:] == "µs":
                        results[row].append(
                            float(duration[:-2]) / 1000)
                    elif duration[-1:] == "s":
                        results[row].append(
                            float(duration[:-1]) * 1000)
                    else:
                        raise ValueError(duration)
    return results


def parse_esp32(dir: str, bench: str, use_pairs=True):
    #serial_ids = ["144460", "1444710", "1444730", "1444740"]
    serial_ids = ["144410", "14430"]
    n = 12
    results = {}

    pairs = [(5, 3), (6, 4), (7, 5), (8, 6),
             (9, 7), (10, 8), (11, 9), (12, 10)]

    if bench == "eval":
        row = "$\mathsf{TDPRF.PartialEval}$"
        results[row] = []
        for serial_id in serial_ids:
            with open("esp32_data/eval_usbserial-{}.txt".format(serial_id)) as f:
                for line in f.readlines():
                    if "Run " in line:
                        duration = line.split(": ")[1][:-1]
                        if duration[-2:] == "ms":
                            results[row].append(float(duration[:-2]))
                        elif duration[-1:] == "s":
                            results[row].append(float(duration[:-1]) * 1000)
                        else:
                            raise ValueError(duration)

        return results

    for i in range(5, n+1):
        for j in range(2, -1, -1):
            if use_pairs and (i, i-j) not in pairs:
                continue
            row = "$\mathsf{}$ $(n = {}, t = {})$".format(
                r"{Gen}" if bench == "init" else r"{Recon}", i, i-j)
            results[row] = []
            for serial_id in serial_ids:
                with open("esp32_data/{}_usbserial-{}_run_{}_{}.txt".format(bench, serial_id, i, i-j)) as f:
                    for line in f.readlines():
                        if "Reconstruct" in line or ("Run " in line and ":" in line):
                            duration = line.split(":")[1][:-1]
                            if duration[-2:] == "ms":
                                results[row].append(float(duration[:-2]))
                            elif duration[-1:] == "s":
                                results[row].append(
                                    float(duration[:-1]) * 1000)
                            else:
                                raise ValueError(duration)
    return results


def parse_ios(dir: str):
    n = 17
    results = {}

    for t in range(n-8, n+1):
        row = "$t = {}$".format(t)
        results[row] = []
        with open("{}/run_{}_{}.txt".format(dir, n, t)) as f:
            for line in f.readlines():
                if "Code generation time" in line:
                    duration = line.split(" ")[-2]
                    results[row].append(
                        float(duration) / 1e6)  # ns
        # First 100 results
        results[row] = results[row][:100]

    return results


def data_to_latex_table(data: dict):
    table = ""
    for row in data:
        row_mean = "${:.2f}$".format(statistics.mean(data[row]))
        row_stdev = "${:.2f}$".format(statistics.stdev(
            data[row])) if len(data[row]) > 1 else r"$\texttt{NA}$"
        table += "{} & {} & {} \\\\\\hline\n".format(row, row_mean, row_stdev)

    template = r"""
\begin{table}[t]
\centering
\caption{Caption}
\begin{tabular}{|l|c|c|}
\hline
"""
    template += "Experiment & $\mu$ & $\sigma$ \\\\\n\hline\hline\n"
    template += table
    template += r"""
\end{tabular}
\label{tab:my_table}
\end{table}
"""

    print(template)


def data_to_boxplot(data: dict, fname: str):
    plt.rcParams.update({
        "text.usetex": True,
        "font.family": "Palatino",
        "font.size": 12
    })

    fig, ax = plt.subplots()
    x_labels = [row for row in data]
    plot_data = [data[row] for row in data]

    ax.boxplot(plot_data, 0, '', 0)
    ax.set(xlabel='Execution Time (ms)')
    ax.set_yticklabels(x_labels)

    plt.tight_layout()
    plt.savefig("{}_box.pdf".format(fname))


# For microbenchmarks only
# Based on https://matplotlib.org/stable/gallery/lines_bars_and_markers/barchart.html
def data_to_grouped_bar(data: dict, fname: str):
    plt.rcParams.update({
        "text.usetex": True,
        "font.family": "Palatino",
        "font.size": 12
    })
    rpi_means = {}
    labels = []
    for row in data:
        rpi = "RPi {}".format(row.split(" ")[1])
        label = row.split("}$ ")[-1]
        if rpi not in rpi_means:
            rpi_means[rpi] = []
        if label not in labels:
            labels.append(label)
        rpi_means[rpi].append(statistics.mean(data[row]))

    x = np.arange(len(labels))  # the label locations
    width = 0.25  # the width of the bars
    multiplier = 0

    fig, ax = plt.subplots(constrained_layout=True)

    for attribute, measurement in rpi_means.items():
        offset = width * multiplier
        rects = ax.bar(x + offset, measurement, width, label=attribute)
        # ax.bar_label(rects, padding=3, rotation=45)
        multiplier += 1

    # Add some text for labels, title and custom x-axis tick labels, etc.
    ax.set_ylabel('Execution time (ms)')
    # ax.set_title('Penguin attributes by species')
    ax.set_xticks(x + width, labels, rotation=45, ha="right")
    ax.legend(loc='upper left', ncols=3)

    # plt.tight_layout()
    plt.savefig("{}_bar.pdf".format(fname))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        "Parse SocIoTy benchmark data and create a table or chart")
    parser.add_argument(
        'directory', choices=["coap_data", "esp32_data", "microbenchmark_data", "mqtt_data", "ios_data"])
    parser.add_argument(
        'output', choices=["latex_table", "boxplot", "bar"])
    parser.add_argument(
        '--bench', choices=["eval", "init", "reconstruct", "nomac"])
    parser.add_argument(
        '--all', action='store_false')
    args = parser.parse_args()

    if args.directory == "esp32_data":
        data = parse_esp32(args.directory, args.bench,
                           use_pairs=args.all)
    elif args.directory == "microbenchmark_data":
        data = parse_microbenchmarks(
            args.directory, args.bench, use_pairs=args.all)
    elif args.directory == "ios_data":
        data = parse_ios(args.directory)
    else:
        data = parse_coap_mqtt(
            args.directory, use_pairs=args.all)

    if args.output == "latex_table":
        data_to_latex_table(data)
    elif args.output == "boxplot":
        data_to_boxplot(data, "{}{}".format(
            args.directory, "_{}".format(args.bench) if args.bench else ""))
    elif args.output == "bar":
        data_to_grouped_bar(data, "{}{}".format(
            args.directory, "_{}".format(args.bench) if args.bench else ""))
