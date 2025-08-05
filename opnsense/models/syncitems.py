from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Syncitems(BaseModel):
    class Meta:
        name = "syncitems"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
