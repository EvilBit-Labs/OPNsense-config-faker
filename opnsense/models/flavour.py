from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Flavour(BaseModel):
    class Meta:
        name = "flavour"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
