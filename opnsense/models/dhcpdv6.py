from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dhcpdv6(BaseModel):
    class Meta:
        name = "dhcpdv6"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
