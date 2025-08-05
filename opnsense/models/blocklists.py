from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Blocklists(BaseModel):
    class Meta:
        name = "blocklists"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
