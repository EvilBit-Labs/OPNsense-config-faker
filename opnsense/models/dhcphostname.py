from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dhcphostname(BaseModel):
    class Meta:
        name = "dhcphostname"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
