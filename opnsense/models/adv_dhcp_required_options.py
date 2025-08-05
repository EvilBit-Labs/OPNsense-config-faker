from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpRequiredOptions(BaseModel):
    class Meta:
        name = "adv_dhcp_required_options"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
