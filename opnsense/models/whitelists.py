from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Whitelists(BaseModel):
    class Meta:
        name = "whitelists"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
