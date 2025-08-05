from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpOptionModifiers(BaseModel):
    class Meta:
        name = "adv_dhcp_option_modifiers"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
