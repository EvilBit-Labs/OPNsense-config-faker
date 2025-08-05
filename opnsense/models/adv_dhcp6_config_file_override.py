from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6ConfigFileOverride(BaseModel):
    class Meta:
        name = "adv_dhcp6_config_file_override"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
