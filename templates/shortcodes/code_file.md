{% set text = load_data(path=path, format="plain") -%}
{% set file_name = path | split(pat="/") | last -%}
[{{ file_name }}]({{ file_name }})
