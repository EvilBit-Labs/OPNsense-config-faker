from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Authorizedkeys(BaseModel):
    class Meta:
        name = "authorizedkeys"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
