Создать в проекте папку `dashboards`. В папке создать файл `config.yml` с содержимым:

```yml
apiVersion: 1

providers:
  - name: dashboards
    type: file
    updateIntervalSeconds: 5
    options:
      path: /etc/grafana/provisioning/dashboards
      foldersFromFilesStructure: true
```

Путь к папке с дашбордами задаётся в файле `.env` в переменной `DASHBOARDS`.

Запусить на выполнение:

```sh
docker compose up
```

Открыть в браузере адрес http://localhost:3000
