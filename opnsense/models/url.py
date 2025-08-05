from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Url(BaseModel):
    class Meta:
        name = "url"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
