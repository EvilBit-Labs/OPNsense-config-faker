from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6ConfigFileOverridePath(BaseModel):
    class Meta:
        name = "adv_dhcp6_config_file_override_path"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
