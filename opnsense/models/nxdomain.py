from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Nxdomain(BaseModel):
    class Meta:
        name = "nxdomain"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
