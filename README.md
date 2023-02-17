# Simple Sensor Report (SSR)

SSR is a sensor data download tool for Hyperview. It allows users to download monthly summary data for one sensor and one asset type. For example, you could use this tool to get energy metering reports for racks. 

```console
$ ssr -t Rack -s averageKwhByHour -m 2 -y 2023 -c "Business Unit" -f ./kwh_rack_report_2023_2.csv
```

SSR has various command line options.

```console
$ ./ssr --help                                                                                                                                        main 
A simple sensor report generator for Hyperview
:w

Usage: ssr [OPTIONS] --asset-type <ASSET_TYPE> --sensor <SENSOR> --year <YEAR> --month <MONTH> --output-file <OUTPUT_FILE>

Options:
  -d, --debug-level <DEBUG_LEVEL>
          Debug level [default: info] [possible values: trace, debug, info, warn, error]
  -t, --asset-type <ASSET_TYPE>
          Asset type. e.g. Rack [possible values: BladeEnclosure, BladeNetwork, BladeServer, BladeStorage, Busway, Camera, Chiller, Crac, Crah, Environmental, FireControlPanel, Generator, InRowCooling, KvmSwitch, Location, Monitor, NetworkDevice, NetworkStorage, NodeServer, PatchPanel, PduAndRpp, PowerMeter, Rack, RackPdu, Server, SmallUps, TransferSwitch, Ups, VirtualServer]
  -s, --sensor <SENSOR>
          Sensor name. E.g. averageKwhByHour
  -c, --custom-property <CUSTOM_PROPERTY>
          Optional custom property name. E.g. "Business Unit"
  -y, --year <YEAR>
          Year value for readings (2020 -> 2029). E.g. 2023
  -m, --month <MONTH>
          Month value for readings (1 -> 12). E.g. 1
  -o, --offset <OFFSET>
          Offset number (0 -> 99999). e.g. 100 [default: 0]
  -l, --limit <LIMIT>
          Record limit (1 -> 250). e.g. 100 [default: 50]
  -f, --output-file <OUTPUT_FILE>
          Name of output csv file. e.g. sensor_data_2023_02.csv
  -h, --help
          Print help
  -V, --version
          Print version
```

# Configuration

A valid Hyperview API client must be used. The API client must have appropriate access to the device sensor data needed. The configuration file must be placed in `$HOME/.ssr/ssr.toml`

## Example

```toml
client_id = 'c33472d0-c66b-4659-a8f8-73c289ba4dbe'
client_secret = '2c239e21-f81b-472b-a8c3-82296d5f250d'
scope = 'HyperviewManagerApi'
auth_url = 'https://example.hyperviewhq.com/connect/authorize'
token_url = 'https://example.hyperviewhq.com/connect/token'
instance_url = 'https://example.hyperviewhq.com'
```

# Defaults
Data for the first 50 assets (ordered by id) is downloaded by default. Sensor data for a maximum of 250 assets can be downloaded at any one time. This can be controlled with the **limit** command line option. More data can be downloaded by using the **offset** command line option to page through assets. 

Using the combination of limit and offset, data from thousands of assets can be downloaded. 

The default debug level is INFO, this provides standard command information. More or less verbose output can be controlled with the **debug-level** command line option.
 
# Limitations

- Only numeric sensors are supported at this time
- Daily summary data can be fetched one month at a time (e.g. 2023-2 for February 2023 data)

