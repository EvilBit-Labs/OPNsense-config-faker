from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpConfigFileOverridePath(BaseModel):
    class Meta:
        name = "adv_dhcp_config_file_override_path"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
