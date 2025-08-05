from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Infrakeepprobing(BaseModel):
    class Meta:
        name = "infrakeepprobing"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
