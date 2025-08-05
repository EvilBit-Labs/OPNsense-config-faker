from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Expires(BaseModel):
    class Meta:
        name = "expires"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
