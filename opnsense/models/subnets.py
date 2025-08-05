from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Subnets(BaseModel):
    class Meta:
        name = "subnets"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
