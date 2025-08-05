from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dhcprejectfrom(BaseModel):
    class Meta:
        name = "dhcprejectfrom"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
