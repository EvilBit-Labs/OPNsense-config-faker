from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpPtRetry(BaseModel):
    class Meta:
        name = "adv_dhcp_pt_retry"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
