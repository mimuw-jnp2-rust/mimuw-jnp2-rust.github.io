{% set text = load_data(path=path, format="plain") -%}
{% set file_name = path | split(pat="/") | last -%}

```{{ language }}
{{ text }}
```

<sub>(Download the source code for this example: [{{ file_name }}]({{ file_name }}))</sub>

