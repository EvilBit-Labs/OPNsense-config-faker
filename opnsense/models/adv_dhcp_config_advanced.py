from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpConfigAdvanced(BaseModel):
    class Meta:
        name = "adv_dhcp_config_advanced"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
