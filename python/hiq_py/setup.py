from setuptools import find_packages, setup
import os
import shutil

setup(
    name='hiq_py',
    version='0.1.0',
    packages=find_packages(include=['hiq_py']),
    include_package_data=True,
    zip_safe=False,
    platform="any",
    install_requires=[
        'hiq_pyfetch',
        'hiq_pydata',
        'hiq_pystrategy',
        'pyecharts',
        'TA-Lib',
    ],
    entry_points={
        'console_scripts': [

        ]
    },
)
