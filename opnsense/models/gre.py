from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Gre(BaseModel):
    class Meta:
        name = "gre"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
