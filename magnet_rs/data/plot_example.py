# This Source Code Form is subject to the terms of the Mozilla Public
# License, v.2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
# Copyright 2021 Peter Dunne

"""Example Python script to plot the field calculated by `magnet_rs`
    Note:
        This requires matplotlib, numpy, and json modules
"""

import json
import numpy as np
import matplotlib.pyplot as plt

def parse_data(data):
    """Parse data dict from magnet_rs

    Args:
        data (dict): structured json dictionary

    Returns:
        (x,y), (Bx, By, Bn) : Tuples containing the x,y coordinates and the magnetic field vector
    """    
    x = np.asarray(data.get('points')['x'],dtype=np.float64)
    y = np.asarray(data.get('points')['y'],dtype=np.float64)

    square_shape = int(np.sqrt(x.shape[0]))
    array_shape = (square_shape, square_shape)

    Bx = np.asarray(data.get('field')['x'],dtype=np.float64)
    By = np.asarray(data.get('field')['y'],dtype=np.float64)

    Bn = np.linalg.norm([Bx, By], axis=0)

    x = x.reshape(array_shape)
    y = y.reshape(array_shape)
    Bx = Bx.reshape(array_shape)
    By = By.reshape(array_shape)
    Bn = Bn.reshape(array_shape)
    
    return (x,y), (Bx, By, Bn)

def parse_json_magnet(file, silent=False):
    """Parse json 

    Args:
        file (string): path to file
        silent (bool, optional): Prints list of magnets in json file. Defaults to False.

    Returns:
        (x,y), (Bx, By, Bn), data : Tuples containing the x,y coordinates and the magnetic field vector, and the dict containing all fields
    """    

    with open(file, "r") as file_object:
        # read file content
        data = json.load(file_object)
        
        if not silent:
            for magnet in data['magnets']:
                print(magnet, '\n')
        
    (x,y), (Bx, By, Bn) = parse_data(data)
    
    
    # Check if file is closed
    if file_object.closed == False:
        print('File is not closed')
    else:
        print('File is closed')
        
    return (x,y), (Bx, By, Bn), data

def plot_res_contour(x, y, Bx, By, Bn, **kwargs):
    """Plots the magnetic field contours

    Args:
        x (ndarray): x coordinates
        y (ndarray): y coordinates
        Bx (nadarray): magnetic field x-component
        By (ndarray): magnetic field y-component
        Bn (ndarray): magnetic field magnitude
    """    
    
    save_fig = kwargs.pop("save_fig", False)
    
    fig, ax = plt.subplots(figsize=(8, 8))

    cmap = "viridis"

    cmin = 0
    cmax = kwargs.pop('cmax', 1.0)
    num_levels = int(np.round((cmax - cmin) / 0.1) + 1)

    
    NQ = 25
    lev2 = np.linspace(cmin, cmax, 256, endpoint=True)

    CS = plt.contourf(
                x,
                y,
                Bn,
                levels=lev2,
                cmap=plt.get_cmap(cmap),
                extend="max",
            )

    lev1 = np.linspace(cmin, cmax, num_levels, endpoint=True)
    _ = plt.contour(
        x,
        y,
        Bn,
        vmin=cmin,
        vmax=cmax,
        levels=lev1,
        linewidths=1.0,
        colors="k",
                )
    CB = plt.colorbar(CS, ticks=lev1, fraction=0.046, pad=0.04)
    CB.ax.tick_params(labelsize=16)
    CB.ax.get_yaxis().labelpad = 20

    CB.ax.set_ylabel(r'$|B|$ (T)', rotation=270, size=18)

    
    
    NPx, NPy = x.shape
    if NQ != 0:
        NSx, NSy = NPx // NQ, NPy // NQ
        with np.errstate(divide="ignore", invalid="ignore"):
            plt.quiver(
                x[::NSx, ::NSy],
                y[::NSx, ::NSy],
                Bx[::NSx, ::NSy] / Bn[::NSx, ::NSy],
                By[::NSx, ::NSy] / Bn[::NSx, ::NSy],
                color='w',
                alpha=1,
            )
    plt.xlabel("x (mm)", size=18)
    plt.ylabel("y (mm)", size=18)
    plt.axis("square")
    ax.tick_params(labelsize=16)

    filename = kwargs.pop("filename", "example")
    if save_fig:
        plt.savefig(filename +'.png', dpi=120, bbox_inches="tight", facecolor='white', transparent=False)
    plt.show()


file = "example_out.json"
(x,y), (Bx, By, Bn), data = parse_json_magnet(file,silent=True)
plot_res_contour(x, y, Bx, By, Bn, cmax=0.7, save_fig=True)