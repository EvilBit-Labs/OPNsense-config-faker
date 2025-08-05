from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class HttpdPassword(BaseModel):
    class Meta:
        name = "httpdPassword"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
