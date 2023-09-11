from setuptools import find_packages, setup
import os
import shutil

setup(
    name='pywq',
    version='0.1.0',
    packages=find_packages(include=['pywq']),
    include_package_data=True,
    zip_safe=False,
    platform="any",
    install_requires=[
        'pywqfetch',
        'pywqdata',
        'pywqstrategy',
        'pyecharts',
        'TA-Lib',
    ],
    entry_points={
        'console_scripts': [

        ]
    },
)
