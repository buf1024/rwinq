# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

import os
import sys
import sphinx_rtd_theme
from recommonmark.transform import AutoStructify
from recommonmark.parser import CommonMarkParser


project = 'pywinq'
copyright = '2023, buf1024@gmail.com'
author = 'buf1024@gmail.com'
release = '1.0.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.todo',
    'sphinx.ext.inheritance_diagram'
]


templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store', '**.ipynb_checkpoints']

language = 'zh_CN'

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# 设置不同后缀的文件使用不同解析器(这个需要后加)
source_suffix = {
    '.rst': 'restructuredtext'
}

# 去掉查看页面源码
html_show_sourcelink = False

# todo插件的设置
todo_include_todos = True

# 主题
extensions.append('sphinx_rtd_theme')

# 使用插件支持markdown
extensions.append('recommonmark')

# 针对`.md`为后缀的文件做markdown渲染
source_suffix[".md"] = 'markdown'

# 设置markdown渲染器的自定义项


def setup(app):
    github_url = 'https://github.com/buf1024/rwinq'
    app.add_config_value(
        "recommonmark_config",
        {
            # "url_resolver": lambda url:  github_url + url,
            # "auto_toc_tree_section": "目录",
            "enable_math": True,  # 支持 math,注意目前inline_math会有问题因此不建议使用.
            'enable_eval_rst': True,  # 支持嵌入rst
        }, True
    )
    app.add_transform(AutoStructify)


# autoapi-python

extensions.append('autoapi.extension')
extensions.append("sphinx.ext.napoleon")
autoapi_type = 'python'
autoapi_dirs = ['../../pywqfetch', '../../pywqdata',]
autoapi_options = ['members', 'undoc-members', 'show-inheritance',
                   'show-module-summary', 'show-inheritance-diagram', 'imported-members']
# autoapi_options = ['members', 'undoc-members', 'show-inheritance',
#                    'show-module-summary', 'special-members', 'show-inheritance-diagram', 'imported-members']
# autoapi_add_toctree_entry = False
