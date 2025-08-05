from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Aliases(BaseModel):
    class Meta:
        name = "aliases"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
