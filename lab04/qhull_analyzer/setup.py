from setuptools import find_packages, setup

setup(
    name="qhull_analyzer",
    version="0.1.0",
    packages=find_packages(),
    install_requires=open("requirements.txt").read().splitlines(),
    entry_points={},
)
