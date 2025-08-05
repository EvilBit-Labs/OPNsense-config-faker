from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Safesearch(BaseModel):
    class Meta:
        name = "safesearch"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
